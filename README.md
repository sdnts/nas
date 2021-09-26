# nas

This is meant to be a self-deployable Network-Attached Storage.

This is still a heavy work in progress, so much so that I would not recommend you use it just yet. It doesn't even have a clever name yet!

### The idea

The idea is to have a zero-config binary that you run on your Pi (or a different low-powered device), that holds all important files. These files are to be accessible on your home's local network. Think of it as a personal Dropbox / Google Drive / iCloud that you control.
The focus is to make it as easy as possible to do this. Documents / Images need to be downloadable and viewable, Video / Music needs to be streamable, and all of it must be possible in a browser with as little Javascript as possible.

Of course, some of this stuff does not work yet; this is a WIP that is meant to keep me busy for a while.

### Deploy it yourself

The steps will likely not change as the project progresses:

1. Run the prebuilt binary (preferably on system startup)
2. Yep

### Features

1. A web-based file explorer that directly mirrors your file system. Upload and organize files and directories just as you would on an external drive.
2. Create multiple user accounts, each with their own, independent file system.
3. Automatically convert any uploaded videos to a format suitable for streaming over HTTP(S)
4. ???

### Shortcomings:

1. Paths are currently hard-coded, which means you cannot change what folder you expose via the server. This obviously has to change as soon as possible.
2. Currently, the only thing standing between your data and intruders is a measly cookie and Basic Authentication. This is fine if the server is only accessible on your local network, but will need more security if you are to open it up to the WWW.
3. Document / Image viewing & downloading currently does not work
4. Other shortcomings that will reveal themselves as I start to use this for myself (as a start)

### Components

There are three components:

1. The `server` is, well, the file server. This does most of the work; you upload / download files to it & it manages user accounts & authentication.
2. `streamgen` is a poorly named module that uses `ffmpeg` to convert a video file into a format suitable for streaming over HTTP.
3. `reaper` is currently empty, but the idea is to only let the server "soft-delete" files / directories. The server moves them to a `Trash` directory somewhere, and `reaper` comes along every once in a while (think `cron`) and permanently deletes things. Hence the name.


### Motivation

I started building this project as a way to explore livestreaming. I wanted to build a non-trivial project to learn Rust, and this turned out to be the perfect way to do so!
Once livestreaming / video-on-demand worked the way I wanted, I decided to expand the scope of the project a little. One thing led to another, and I ended up with a plan to replace my iCloud Drive with this. (Little did I realize, I was re-inventing FTP; but oh well)

### Concept screenshots

![Auth](https://user-images.githubusercontent.com/7689783/134803695-4a7ee2e5-cd72-4a47-8ae4-cef239fc0906.png)
![FS](https://user-images.githubusercontent.com/7689783/134803708-3771b9fc-4125-4756-bcb3-cff6e20e892b.png)
![FS-1](https://user-images.githubusercontent.com/7689783/134803706-363e3923-572d-4f7b-a3e5-42f46f5661c3.png)
![Stream](https://user-images.githubusercontent.com/7689783/134803712-fe405fb2-8a27-4767-b04f-47bfb2796b13.png)
