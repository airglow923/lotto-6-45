#!/usr/bin/env python3

import itertools
import warnings
import sys
import locale
from collections.abc import Callable
from typing import List, TypeVar, Optional
from math import factorial


localization = {
    "en_US": {
        "init-values-msg": "Enter values to fix. (x if none, q to quit): ",
        "values-empty-err": "Press enter key only if you provide the values.\n",
        "values-requirement-err": "Enter natural numbers where n > 1 and 1 < 46.\n",
        "print-success-msg": "\nPrint success\n",
        "exclude-msg": "Enter numbers to exclude. (q if none): ",
    },
    "ko_KR": {
        "init-values-msg": "고정할 값을 입력하시오. (없으면 x, 종료하려면 q): ",
        "values-empty-err": "값을 입력하고 엔터를 누르시오.\n",
        "values-requirement-err": "n > 1 이고 n < 46 인 자연수만 입력하시오.\n",
        "print-success-msg": "\n출력완료\n",
        "exclude-msg": "제외할 수가 있다면 입력하시오. (없으면 q): ",
    },
}


def main(_argv):
    """
    locale.getdefaultlocale is deprecated and will be removed in 3.15:

        https://docs.python.org/3/library/locale.html#locale.getdefaultlocale

    but locale.getlocale sucks due to:

        https://github.com/python/cpython/issues/82986
    """
    warnings.filterwarnings("ignore", category=DeprecationWarning)

    user_locale = locale.getdefaultlocale()[0]

    try:
        translation = localization[str(user_locale)]
    except:
        translation = localization["en_US"]

    while True:
        user_input = input(translation["init-values-msg"]).strip()
        set_fixed = set()

        if not user_input:
            print(translation["values-empty-err"])
            continue

        if "q" in user_input:
            break

        if "x" not in user_input:
            set_fixed = split_parse_ints(user_input, lambda n: n >= 1 and n <= 45)

        if set_fixed is None:
            print(translation["values-requirement-err"])
            continue

        combs = get_combinations([*range(1, 46)], list(set_fixed), 6)

        for i, comb in enumerate(combs):
            print("{:>7}: {}".format(i, comb))

        print(translation["print-success-msg"])

        while True:
            exclusion = input(translation["exclude-msg"]).strip()

            if not exclusion:
                print(translation["values-empty-err"])
                continue

            if "q" in exclusion:
                break

            excludes = split_parse_ints(exclusion)

            if excludes is None:
                print(translation["values-requirement-err"])
                continue

            for exclude in excludes:
                combs = list(filter(lambda comb: exclude not in comb, combs))

            for i, comb in enumerate(combs):
                print("{:>7}: {}".format(i, comb))

            print(translation["print-success-msg"])


def split_parse_ints(
    s: str, predicate: Optional[Callable[[int], bool]] = None
) -> Optional[List[int]]:
    res = []
    predicate = predicate if predicate else lambda _: True

    for i in s.split():
        try:
            n = int(i)

            if not predicate(n):
                raise ValueError

            res.append(n)

        except ValueError:
            return None

    return res


def binomial(x, y):
    try:
        return factorial(x) // factorial(y) // factorial(x - y)
    except ValueError:
        return 0


T = TypeVar("T")


def get_combinations(
    set_target: List[T], set_fixed: List[T], n_seq: int
) -> List[List[T]]:
    n_set_target = len(set_target)
    n_set_fixed = len(set_fixed)

    if n_set_target == 0 or n_seq <= n_set_fixed:
        print(
            "set cannot be empty and n_seq should be greater than length of set_fixed"
        )
        return []

    n_rand = n_seq - n_set_fixed
    # n_combs = binomial(n_set_target, n_rand)
    combs = []
    set_filtered = set(set_target) - set(set_fixed)

    for set_comb in itertools.combinations(set_filtered, n_rand):
        comb = []

        for e in set_fixed:
            comb.append(e)

        for e in set_comb:
            comb.append(e)

        combs.append(comb)

    return combs


if __name__ == "__main__":
    main(sys.argv[1:])
else:
    print("This script must be executed directly.")
    sys.exit(1)
