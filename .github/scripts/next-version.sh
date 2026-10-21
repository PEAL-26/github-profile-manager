#!/usr/bin/env bash
# Calcula a próxima versão incremental (semver) com base na última tag `v*`
# e nas mensagens de commit desde então (Conventional Commits):
#   - BREAKING CHANGE ou tipo!:  major
#   - feat:                      minor
#   - resto (fix, perf, etc.):   patch
#
# A tag é criada no GitHub Actions pelo softprops/action-gh-release, por isso
# este script apenas calcula e expõe o valor (via GITHUB_OUTPUT quando presente).
set -euo pipefail

cd "$(dirname "$0")/../.."

current() {
  if [ -n "$(git tag -l 'v*' --sort=-v:refname | head -n1)" ]; then
    git tag -l 'v*' --sort=-v:refname | head -n1 | sed 's/^v//'
  else
    node -p "require('./project/package.json').version"
  fi
}

commits_since_last_tag() {
  local last
  last="$(git tag -l 'v*' --sort=-v:refname | head -n1 || true)"
  # Inclui o corpo das mensagens para detetar "BREAKING CHANGE:".
  if [ -z "$last" ]; then
    git log --pretty=format:%s%n%b
  else
    git log --pretty=format:%s%n%b "${last}..HEAD"
  fi
}

bump() {
  local version="$1" level="$2"
  local major minor patch
  IFS=. read -r major minor patch <<<"$version"
  case "$level" in
    major) echo "$((major + 1)).0.0" ;;
    minor) echo "${major}.$((minor + 1)).0" ;;
    patch) echo "${major}.${minor}.$((patch + 1))" ;;
  esac
}

determine_level() {
  local msgs="$1"
  if echo "$msgs" | grep -qE '^[a-zA-Z]+(\([^)]*\))?!:|BREAKING CHANGE'; then
    echo major
  elif echo "$msgs" | grep -qE '^feat(\(|:| )'; then
    echo minor
  else
    echo patch
  fi
}

CURRENT="$(current)"
LEVEL="$(determine_level "$(commits_since_last_tag)")"
NEXT="$(bump "$CURRENT" "$LEVEL")"

echo "Versão atual: v${CURRENT} | Nível: ${LEVEL} | Próxima: v${NEXT}"

# Compatível com GitHub Actions
if [ -n "${GITHUB_OUTPUT:-}" ]; then
  echo "version=${NEXT}" >>"$GITHUB_OUTPUT"
  echo "tag=v${NEXT}" >>"$GITHUB_OUTPUT"
fi
