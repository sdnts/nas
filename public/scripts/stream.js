(() => {
  const video = document.getElementById("video");

  if (Hls.isSupported()) {
    const hls = new Hls({ debug: true });

    hls.loadSource(window.streamSrc);
    hls.attachMedia(video);

    hls.on(Hls.Events.MEDIA_ATTACHED, function () {
      //   video.play();
    });

    hls.on(Hls.Events.ERROR, function (event, data) {
      console.log("Error in stream", event, data);
    });
  } else if (video.canPlayType("application/vnd.apple.mpegurl")) {
    video.src = window.streamSrc;
    video.addEventListener("canplay", function () {
      video.play();
    });
  }
})();
