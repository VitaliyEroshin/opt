import os
import subprocess
from datetime import datetime
import argparse
from ortools.sat.python import cp_model

class Variable:
    def __init__(self, number, negate):
        self.number = number
        self.negate = negate

    def call(self, arr):
        return bool(arr[self.number]) ^ self.negate


class Sat:
    def __init__(self, size, clauses):
        self.size = size
        self.clauses = clauses

    def call(self, arr):
        return all([any([x.call(arr) for x in cl]) for cl in self.clauses])


class CpSatSolver:
    def solve(self, sat):
        v = dict()
        model = cp_model.CpModel()

        for clause in sat.clauses:
            for var in clause:
                if var.number in v:
                    continue

                i = var.number

                v[i] = model.NewIntVar(0, 1, "a" + str(i))
                v[-i] = model.NewIntVar(0, 1, "b" + str(i))

                model.Add(v[i] + v[-i] <= 1)
                model.Add(v[i] + v[-i] >= 1)

            a = [var.number * (-1) ** var.negate for var in clause]
            model.Add(v[a[0]] + v[a[1]] + v[a[2]] >= 1)

        solver = cp_model.CpSolver()
        status = solver.Solve(model)

        if status == cp_model.INFEASIBLE:
            return "Error: Unsatisfiable"

        eval_set = []

        for i in v:
            if i == 0:
                continue

            eval_set.append(str(i * (-1) ** (not bool(solver.Value(v[i])))))

        return " ".join(map(str, eval_set))


class OptExternalSatSolver:
    def __init__(self, solver_name):
        self.solver_name = solver_name

    def solve(self, sat):
        p = subprocess.Popen(
            ['cargo', 'run', '--bin', f'test_{self.solver_name}_solver'],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL
        )

        p.communicate(("\n".join(get_printable_sat(sat))).encode())
        p.wait()

        stdout, stderr = p.communicate()
        output = stdout.decode()
        return output


def resolve_testcases(path):
    folder = os.fsencode(path)

    filenames = []

    for file in os.listdir(folder):
        filename = os.fsdecode(file)
        if filename.endswith(('.cnf', '.txt')):
            filenames.append(filename)

    return filenames


def parse_testcase(path):
    lines = []
    with open(path, 'r') as f:
        lines =  f.readlines()

    clauses = []
    num_clauses = 0
    num_variables = 0

    for line in lines:
        if len(line) == 0 or line[0] == 'c':
            continue

        if line[0] == 'p':
            p, cnf, n, m = line.split()
            num_variables = int(n)
            num_clauses = int(m)
            continue
        
        if line[0] == '%':
            break

        literals = map(int, line.split())

        clause = []
        for l in literals:
            if l == 0:
                break

            number = abs(l)
            negate = (l < 0)
            clause.append(Variable(number, negate))
        
        clauses.append(clause)

    assert len(clauses) == num_clauses

    return Sat(num_variables, clauses)


def get_printable_sat(sat):
    lines = []
    lines.append(f"{len(sat.clauses)}")
    for clause in sat.clauses:
        printable_clause = [str(v.number * (-1) ** v.negate) for v in clause]
        lines.append(" ".join(printable_clause))
    return lines


def print_colorful_text(text, color):
    print(f"{color}{text}\033[0m", end='')


def check_testcase(cnf, output, time_elapsed, testcase_name=''):
    def print_fail_feedback(feedback, testcase_name=''):
        print_colorful_text("[FAIL] ", '\033[1;91m')
        print(f"({testcase_name}) {feedback}")

    if len(output) == 0:
        print_fail_feedback("Solver output is empty", testcase_name)
        return False
    
    if output[0] == 'E':
        print_fail_feedback(output.rstrip(), testcase_name)
        return False
    
    values = list(map(int, output.split()))

    eval_set = [0] * (cnf.size + 1)

    for v in values:
        if v > 0:
            eval_set[v] = 1

    if cnf.call(eval_set) == False:
        print_fail_feedback("Wrong evaluation set", testcase_name)
        return False

    print_colorful_text("[PASS] ", '\033[1;92m')
   
    print(f"{testcase_name} successfully solved! {time_elapsed}")
    return True

def run_testcase(cnf, solver, testcase_name=''):
    timer_start = datetime.now()

    output = solver.solve(cnf)

    timer_end = datetime.now()
    time_elapsed = timer_end - timer_start

    return check_testcase(cnf, output, time_elapsed, testcase_name)

def test_solver(tc_path, solver):
    path = tc_path
    testcases = resolve_testcases(path)
    testcases = sorted(testcases)

    for tc in testcases:
        cnf = parse_testcase(path + '/' + tc)
        run_testcase(cnf, solver, tc)


def main():
    parser = argparse.ArgumentParser(description='Opt utility tool. Github: https://github.com/VitaliyEroshin/opt')
    parser.add_argument('--solver', action='store', default=None)
    parser.add_argument('--testcases', action='store', default='./testcases')
    parser.add_argument('--dot_cnf_path', action='store', default=None)

    args = vars(parser.parse_args())

    if args['solver'] == "python_cp":
        solver = CpSatSolver()
        test_solver(args['testcases'], solver)
        return

    if args['solver'] is not None:
        solver = OptExternalSatSolver(args['solver'])
        test_solver(args['testcases'], solver)
        return
    
    if args['dot_cnf_path'] is not None:
        path = args['dot_cnf_path']
        
        cnf = parse_testcase(path)
        print("\n".join(get_printable_sat(cnf)))
        return
    
    parser.print_help()

main()