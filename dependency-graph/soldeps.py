#!/usr/bin/env python3

import argparse
import re
import subprocess

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--solana-root', dest='solana_root', metavar='PATH', default=None, help='PATH to the Solana project')
    parser.add_argument('--include', dest='include_file', metavar='FILE', help='FILE containing a list of modules to include (can use a regex)', default=None)
    parser.add_argument('--exclude', dest='exclude_file', metavar='FILE', help='FILE containing a list of modules to exclude', default=None)
    parser.add_argument('--output', dest='output', metavar='FILE', help='FILE to write the output to')

    args = parser.parse_args()

    # Check that the solana directory exists
    if not args.solana_root:
        parser.error('Cannot find the solana root project. Please use --solana-root option.')

    # Collect includes
    includes = []
    if args.include_file:
        with open(args.include_file) as inputFile:
            includes = [re.compile(f'^{line}$') for line in (l.strip() for l in inputFile.read().split('\n')) if len(line) and not line.startswith('#')]
    else:
        includes = [re.compile('.*')]
    
    # Collect excludes
    excludes = []
    if args.exclude_file:
        with open(args.exclude_file) as inputFile:
            excludes = [re.compile(f'^{line}$') for line in (l.strip() for l in inputFile.read().split('\n')) if len(line) and not line.startswith('#')]

    # Collect actual project modules
    result = subprocess.run(['cargo', 'depgraph'], cwd=args.solana_root, stdout=subprocess.PIPE)
    gvz = result.stdout.decode('utf-8')
    regex = re.compile('.*label = "(?P<module>[^"]+)".*')
    all_modules = set()
    for line in gvz.split('\n'):
        match = regex.match(line)
        if match:
            module = match.group('module').split(' ', 1)[0]
            all_modules.add(module)

    # Check for consistency
    unknown_includes = sorted([element for element in includes if not any(element.match(module) for module in all_modules)])
    if unknown_includes:
        parser.error(f'The following include modules are not found in the project: {",".join(include[1:-1] for include in unknown_includes)}')

    unknown_excludes = sorted([element for element in excludes if not any(element.match(module) for module in all_modules)])
    if unknown_excludes:
        parser.error(f'The following exclude modules are not found in the project: {",".join(unknown_excludes)}')

    # Prune the graph
    node_pattern = re.compile(r'\s+(?P<node_id>\d+)\s+\[\s+label = "(?P<module>[^"]+)".*')
    edge_pattern = re.compile(r'\s+(?P<from_id>\d+)\s+->\s+(?P<to_id>\d+).*')

    lines = []
    module_to_nodeid = {}
    nodeid_to_module = {}
    nodeid_remap = {}
    for line in gvz.split('\n'):
        match = node_pattern.match(line)
        if match:
            node_id = match.group('node_id')
            module_old = match.group('module')
            module = module_old.split(' ',1)[0]
            
            module_to_nodeid[module] = node_id
            nodeid_to_module[node_id] = module

            if any(element.match(module) for element in includes) and not any(element.match(module) for element in excludes): 
                lines.append(line.replace(module_old, module))
        else:
            match = edge_pattern.match(line)
            if match:
                from_id = match.group('from_id')
                to_id = match.group('to_id')
                for node_id in (from_id, to_id):
                    module = nodeid_to_module.get(node_id, None)
                    if module is not None:

                        if not any(element.match(module) for element in includes):
                            break
                        if any(element.match(module) for element in excludes): 
                            break
                else:
                    lines.append(line)
            else:
                lines.append(line)
    
    gvz = '\n'.join(lines)
    if args.output:
        with open(args.output, 'w') as outputFile:
            outputFile.write(gvz)
    else:
        print(gvz)
