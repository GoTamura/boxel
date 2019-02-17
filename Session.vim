let SessionLoad = 1
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd ~/Documents/rust/game01
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +118 src/client.rs
badd +96 src/server.rs
badd +133 ~/Documents/rust/tokio_test/src/main.rs
badd +14 Cargo.toml
badd +0 todo.md
argglobal
silent! argdel *
$argadd src/client.rs
edit src/server.rs
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd _ | wincmd |
split
1wincmd k
wincmd w
wincmd w
set nosplitbelow
set nosplitright
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe '1resize ' . ((&lines * 12 + 13) / 27)
exe 'vert 1resize ' . ((&columns * 110 + 75) / 151)
exe '2resize ' . ((&lines * 12 + 13) / 27)
exe 'vert 2resize ' . ((&columns * 110 + 75) / 151)
exe 'vert 3resize ' . ((&columns * 40 + 75) / 151)
argglobal
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 199 - ((4 * winheight(0) + 6) / 12)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
199
normal! 0
wincmd w
argglobal
if bufexists('todo.md') | buffer todo.md | else | edit todo.md | endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 37 - ((11 * winheight(0) + 6) / 12)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
37
normal! 015|
wincmd w
argglobal
enew
file __Tagbar__.1
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal nofen
wincmd w
2wincmd w
exe '1resize ' . ((&lines * 12 + 13) / 27)
exe 'vert 1resize ' . ((&columns * 110 + 75) / 151)
exe '2resize ' . ((&lines * 12 + 13) / 27)
exe 'vert 2resize ' . ((&columns * 110 + 75) / 151)
exe 'vert 3resize ' . ((&columns * 40 + 75) / 151)
tabnext 1
if exists('s:wipebuf') && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 winminheight=1 winminwidth=1 shortmess=filnxtToOFI
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
