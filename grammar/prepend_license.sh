# SPDX-License-Identifier: Apache-2.0

for f in $1/*.py; do
    echo '# SPDX-License-Identifier: Apache-2.0' | cat - $f > temp && mv temp $f
done