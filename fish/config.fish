if status is-interactive
    # Commands to run in interactive sessions can go here
end

starship init fish | source
alias ls='lsd'
alias la='lsd -Fla'
set PATH $PATH ~/.cargo/bin
alias rofi='/home/purnab/.config/rofi/launchers/type-7/launcher.sh'
alias powermenu='/home/purnab/.config/rofi/powermenu/type-1/powermenu.sh'
