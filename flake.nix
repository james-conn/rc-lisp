{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/release-24.11";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem (system:
			let pkgs = import nixpkgs {
				inherit system;
			}; in {
				devShell = pkgs.mkShell {
					packages = with pkgs; [
						cargo
						clippy
					];
				};
		});
}
