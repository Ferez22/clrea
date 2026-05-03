# clrea zsh integration. Source from .zshrc:  eval "$(clreactl init zsh)"
unsetopt correct correct_all 2>/dev/null

command_not_found_handler() {
  emulate -L zsh
  local typo="$1"
  shift
  local args="$@"

  local out
  out="$(clreactl suggest "$typo" 2>/dev/null)"
  local code=$?

  if (( code != 1 )); then
    print -r -- "zsh: command not found: $typo" >&2
    return 127
  fi

  local correct="${out#*$'\t'}"

  # Red bold text + emojis. ⏎ = Enter to confirm, ⎋ = Esc to cancel.
  print -Pn -- "%F{red}🔴 clrea:%f %F{red}%Bdid you mean%b%f %F{white}%B$correct%b%f %F{red}?%f  %F{8}[⏎ confirm · ⎋ cancel]%f "

  local key
  # -k 1 = one keypress, -s = silent (don't echo)
  read -sk 1 key

  case $key in
    $'\n'|$'\r'|'')
      print -P -- "%F{green}⚪ → $correct%f"
      clreactl learn "$typo" "$correct" >/dev/null 2>&1
      "$correct" ${=args}
      return $?
      ;;
    *)
      print -P -- "%F{8}⚪ cancelled%f"
      return 127
      ;;
  esac
}
