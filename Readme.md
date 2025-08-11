<div align="center">
<img src="./img/bulb.jpg" width="75" ><br/>
<img src="./img/cuddlyferris.png" width="200">
</div>

## Rustylights

It's like [funkenlights](https://github.com/stefanSchinkel/funkenlights), just a bit more rusty :no_mouth:

### development

You'll need to install a build environment for ARM.On eg. Ubuntu, this works:
```sh
sudo apt install gcc-arm-linux-gnueabihf
```
and the you should be able to
```sh
cargo build --target armv7-unknown-linux-musleabihf
```

and have an ARM binary
