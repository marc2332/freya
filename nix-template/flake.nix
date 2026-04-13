{
  description = "A flake with the freya devshells";
  inputs.freya.url = "github:marc2332/freya";

  outputs =
    {
      freya,
      ...
    }:
    freya;
}
