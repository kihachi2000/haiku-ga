#!/bin/bash

OPT=(
    "-it"
    "--rm"
    "-v" "/home/kem/git/haiku-ga:/home/kem/git/haiku-ga"
    "--workdir" "/home/kem/git/haiku-ga"
)

# Mac以外のときはユーザーidを変更する
# https://qiita.com/s10akir/items/19e130682204e4dbbf3b
if [ "$(uname)" != "Darwin" ]; then
    OPT+=(
        "-u" "$(id -u):$(id -g)"

        # read onlyで/etc/passwdと/etc/groupをボリューム
        # https://qiita.com/yohm/items/047b2e68d008ebb0f001
        "-v" "/etc/group:/etc/group:ro"
        "-v" "/etc/passwd:/etc/passwd:ro"
    )
fi

docker run "${OPT[@]}" rust:1.87 cargo "$@"
