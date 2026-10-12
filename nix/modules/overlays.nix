{ inputs, ... }:
{
  flake.overlays.default =
    final: _:
    let
      mkZed = import ../toolchain.nix { inherit inputs; };
    in
    {
      kurocto-editor = mkZed final;
    };
}
