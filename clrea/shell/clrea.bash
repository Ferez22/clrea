# clrea bash integration. Source from .bashrc:  eval "$(clreactl init bash)"
command_not_found_handle() {
  local typo="$1"
  shift
  local out code correct
  out="$(clreactl suggest "$typo" 2>/dev/null)"
  code=$?
  if (( code != 1 )); then
    printf 'bash: %s: command not found\n' "$typo" >&2
    return 127
  fi
  correct="${out#*$'\t'}"

  local R=$'\033[1;31m' D=$'\033[2m' G=$'\033[32m' N=$'\033[0m'
  printf '%s🔴 clrea:%s %sdid you mean%s %s%s%s %s?%s  %s[Enter confirm · Esc cancel]%s ' \
    "$R" "$N" "$R" "$N" "$R" "$correct" "$N" "$R" "$N" "$D" "$N"

  local key
  IFS= read -rsn1 key
  case "$key" in
    ''|$'\n'|$'\r')
      printf '%s⚪ → %s%s\n' "$G" "$correct" "$N"
      clreactl learn "$typo" "$correct" >/dev/null 2>&1
      "$correct" "$@"
      return $?
      ;;
    *)
      printf '%s⚪ cancelled%s\n' "$D" "$N"
      return 127
      ;;
  esac
}
