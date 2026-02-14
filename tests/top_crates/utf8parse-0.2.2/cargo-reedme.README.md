A table-driven UTF-8 Parser

This module implements a table-driven UTF-8 parser which should
theoretically contain the minimal number of branches (1). The only branch is
on the `Action` returned from unpacking a transition.