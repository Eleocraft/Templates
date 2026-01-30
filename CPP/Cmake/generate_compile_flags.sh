echo "CompileFlags:" > .clangd
echo "  Add:" >> .clangd
clang -v -xc++ /dev/null -fsyntax-only </dev/null 2>&1 \
  | sed -n '/#include <...> search starts here:/, /End of search list./p' \
  | tail -n +2 | head -n -1 \
  | sed 's/^/    - -isystem\n    - /' >> .clangd
