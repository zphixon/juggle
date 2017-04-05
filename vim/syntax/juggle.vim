" Vim syntax file
" Language: juggle

if exists("b:current_syntax")
    finish
endif

syn case match

syn keyword stacks toss catch rethrow recatch drop turn
syn keyword math plus minus times divided modulo
syn keyword array append nth
syn keyword io curse joke feedback
syn keyword bool true false and or not
syn keyword routine routine
syn keyword flow if while else end
syn keyword comparison equal greater lesser

syn match number '\d\+'
syn match number '[+-]\d\+'
syn match comment '#.*$'

let b:current_syntax = 'juggle'

hi def link number Constant
hi def link bool Constant
hi def link flow Conditional
hi def link io Function
hi def link math Function
hi def link array Function
hi def link routine Function
hi def link comparison Function
hi def link stacks Identifier
hi def link comment Comment

