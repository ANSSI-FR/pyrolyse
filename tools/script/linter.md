

# Python

yapf -i *.py
pylint *.py


# Shell

NB: We do not use SC2038 because it yields some problems when we use find on a directory and it suggests to use -print0.

shellcheck -e SC2038 *.sh



