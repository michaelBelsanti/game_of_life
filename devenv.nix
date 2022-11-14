{ pkgs, ... }:

{
  packages = with pkgs; [ 
    pkgs.rust-analyzer
  ];

  enterShell = ''
    zsh
  '';

  # https://devenv.sh/languages/
  languages.rust.enable = true;
}
