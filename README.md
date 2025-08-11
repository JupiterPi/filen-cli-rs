# filen-cli-rs

This is a Rust rewrite of [FilenCloudDienste/filen-cli](https://github.com/FilenCloudDienste/filen-cli) using the [Filen Rust SDK](https://github.com/FilenCloudDienste/filen-rs).

> [!Warning]
> This project is in active development and still missing most functionality. **Don't use it.**

It aims to improve the Filen CLI in multiple ways:
- Significantly reduced download size from up to 130MB to ~5MB. 
- Significantly improved performance, especially startup performance, which is critical for CLI applications (because we're using Rust instead of TypeScript). 


### Development

Make sure that [FilenCloudDienste/filen-rs](https://github.com/FilenCloudDienste/filen-rs) is available locally at `../filen-rs`.