<template lang="pug">
.li-media-player
  audio.li-audio-element
    source(src='https://tile.loc.gov/storage-services/media/ls/sagan/1958124-3-1.mp3')
  //- video.li-video-element
  //-   source(src='http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4')
  .li-media-player-controls
    .li-control-row
      .li-time
        .li-time-current 0:00
      .li-time-control
        .li-buffered
        .li-progress
        .li-duration-handler
        .li-current-time
    .li-control-row
      co-button.li-play-pause
        svg(version='1.1', xmlns='http://www.w3.org/2000/svg', xmlns:xlink='http://www.w3.org/1999/xlink', width='20', height='20', viewBox='0 0 20 20')
          path(d='M18.5 20h-17c-0.827 0-1.5-0.673-1.5-1.5v-17c0-0.827 0.673-1.5 1.5-1.5h17c0.827 0 1.5 0.673 1.5 1.5v17c0 0.827-0.673 1.5-1.5 1.5zM1.5 1c-0.276 0-0.5 0.224-0.5 0.5v17c0 0.276 0.224 0.5 0.5 0.5h17c0.276 0 0.5-0.224 0.5-0.5v-17c0-0.276-0.224-0.5-0.5-0.5h-17z')
          path(d='M6.5 16c-0.080 0-0.16-0.019-0.233-0.057-0.164-0.086-0.267-0.257-0.267-0.442v-11c0-0.186 0.103-0.356 0.267-0.442s0.363-0.075 0.516 0.031l8 5.5c0.136 0.093 0.217 0.247 0.217 0.412s-0.081 0.319-0.217 0.412l-8 5.5c-0.085 0.058-0.184 0.088-0.283 0.088zM7 5.451v9.099l6.617-4.549-6.617-4.549z')
      .li-player-volume
        co-button.li-player-mute
          svg(version='1.1', xmlns='http://www.w3.org/2000/svg', xmlns:xlink='http://www.w3.org/1999/xlink', width=20, height=20, viewBox='0 0 20 20')
            path(d='M10.834 5.128c-0.205-0.185-0.521-0.168-0.706 0.037l-1.128 1.254v-2.169c0-0.72-0.427-1.043-0.85-1.043-0.247 0-0.494 0.104-0.736 0.309l-4.098 3.483h-1.816c-0.827 0-1.5 0.673-1.5 1.5v4c0 0.827 0.673 1.5 1.5 1.5h0.677l-1.049 1.166c-0.185 0.205-0.168 0.521 0.037 0.706 0.096 0.086 0.215 0.128 0.334 0.128 0.137 0 0.273-0.056 0.372-0.166l1.562-1.735 3.981 3.384c0.241 0.205 0.489 0.309 0.736 0.309 0 0 0 0 0 0 0.272 0 0.516-0.132 0.67-0.362 0.119-0.179 0.18-0.408 0.18-0.68v-8.836l1.872-2.080c0.185-0.205 0.168-0.521-0.037-0.706zM8 4.331v3.199l-4 4.444v-4.244l4-3.4zM1 12.5v-4c0-0.276 0.224-0.5 0.5-0.5h1.5v5h-1.5c-0.276 0-0.5-0.224-0.5-0.5zM8 16.669l-3.898-3.313 3.898-4.331v7.644z', fill='#ffffff')
        .li-volume-menu
          .li-volume-total
            .li-volume-amount
            .li-volume-handler
      co-button.li-control-fullscreen
        svg(version='1.1', xmlns='http://www.w3.org/2000/svg', xmlns:xlink='http://www.w3.org/1999/xlink', width='20', height='20', viewBox='0 0 20 20')
          path(d='M17.5 18h-16c-0.827 0-1.5-0.673-1.5-1.5v-12c0-0.827 0.673-1.5 1.5-1.5h16c0.827 0 1.5 0.673 1.5 1.5v12c0 0.827-0.673 1.5-1.5 1.5zM1.5 4c-0.276 0-0.5 0.224-0.5 0.5v12c0 0.276 0.224 0.5 0.5 0.5h16c0.276 0 0.5-0.224 0.5-0.5v-12c0-0.276-0.224-0.5-0.5-0.5h-16z')
          path(d='M11.854 6.646l-2-2c-0.195-0.195-0.512-0.195-0.707 0l-2 2c-0.195 0.195-0.195 0.512 0 0.707s0.512 0.195 0.707 0l1.146-1.146v2.293c0 0.276 0.224 0.5 0.5 0.5s0.5-0.224 0.5-0.5v-2.293l1.146 1.146c0.098 0.098 0.226 0.146 0.354 0.146s0.256-0.049 0.354-0.146c0.195-0.195 0.195-0.512 0-0.707z')
          path(d='M11.854 13.646c-0.195-0.195-0.512-0.195-0.707 0l-1.146 1.146v-2.293c0-0.276-0.224-0.5-0.5-0.5s-0.5 0.224-0.5 0.5v2.293l-1.146-1.146c-0.195-0.195-0.512-0.195-0.707 0s-0.195 0.512 0 0.707l2 2c0.098 0.098 0.226 0.146 0.354 0.146s0.256-0.049 0.354-0.146l2-2c0.195-0.195 0.195-0.512-0-0.707z')
          path(d='M17.354 10.146l-2-2c-0.195-0.195-0.512-0.195-0.707 0s-0.195 0.512 0 0.707l1.146 1.146h-2.293c-0.276 0-0.5 0.224-0.5 0.5s0.224 0.5 0.5 0.5h2.293l-1.146 1.146c-0.195 0.195-0.195 0.512 0 0.707 0.098 0.098 0.226 0.146 0.354 0.146s0.256-0.049 0.354-0.146l2-2c0.195-0.195 0.195-0.512 0-0.707z')
          path(d='M5.5 10h-2.293l1.146-1.146c0.195-0.195 0.195-0.512 0-0.707s-0.512-0.195-0.707 0l-2 2c-0.195 0.195-0.195 0.512 0 0.707l2 2c0.098 0.098 0.226 0.146 0.354 0.146s0.256-0.049 0.354-0.146c0.195-0.195 0.195-0.512 0-0.707l-1.146-1.146h2.293c0.276 0 0.5-0.224 0.5-0.5s-0.224-0.5-0.5-0.5z')

</template>

<script lang="ts">

import { defineComponent } from 'vue';
import coButton from './controls/co-button.vue';

export default defineComponent({
  name: 'coMedia',
  components: {
    'co-button': coButton
  }
});
// let currentlyPlaying = false;
// let mediaElement = document.querySelector('.li-audio-element');
// let progress = document.querySelector('.li-progress');
// let currentTimeElement = document.querySelector('.li-current-time');
// let playPauseButton = document.querySelector('.li-play-pause');
// let durationElement = document.querySelector('.li-duration-handler');
// let duration;
// let timeCurrent = document.querySelector('.li-time-current');
// let playerMute = document.querySelector('.li-player-mute');
// let playerVolumeAmount = document.querySelector('.li-volume-amount');
// let playerVolumeHandler = document.querySelector('.li-volume-handler');

// if (playerMute) {
//   playerMute.addEventListener('click', function() {
//     if(mediaElement.muted) {
//       mediaElement.muted = false;
//     } else {
//       mediaElement.muted = true;
//     }
//   });
// }

// if (playerVolumeAmount && playerVolumeHandler) {
//   playerVolumeHandler.addEventListener('click', function(event) {
//     let volumeHandlerWidth = playerVolumeHandler.offsetWidth;
//     console.log(volumeHandlerWidth);
//     let volumeAmount = event.offsetX / volumeHandlerWidth;
//     console.log(volumeAmount);
//     playerVolumeAmount.style.width = volumeAmount * 100 + '%';
//     mediaElement.volume = volumeAmount;
//   });
// }

// let media = {
//   id: '',
//   fingerprint: '',
//   src: '',
//   duration: 0,
//   playing: false,
//   loadStatus: '',
//   tracks: [
//     {
//       type: 'video',
//       encoder: 'h264'
//     },
//     {
//       type: 'audio',
//       encoder: 'mp3'
//     }
//   ],
//   meta: {
//     name: '',
//     description: '',
//     authors: ''
//   }
// }

// console.log(mediaElement.buffered);


// if (playPauseButton) {
//   playPauseButton.addEventListener('click', function() {
//     if (currentlyPlaying === false) {
//       mediaElement.play();
//       console.log('Played media');
//       currentlyPlaying = true;
//     } else if (currentlyPlaying === true) {
//       mediaElement.pause();
//       console.log('Paused media');
//       currentlyPlaying = false;
//     }


//   });
// }

// if (mediaElement) {
//   mediaElement.addEventListener('timeupdate', updateProgress);
//   duration = mediaElement.duration;
//   console.log(duration);
// }

// function updateProgress() {
//   if ( progress && currentTimeElement ) {
//     let currentTime = mediaElement.currentTime;
//     let progressWidth = (currentTime / duration) * 100 + '%';
//     console.log(currentTime);
//     console.log(progressWidth);
//     progress.style.width = progressWidth;
//     currentTimeElement.style.left = progressWidth;
//     let timeMinutes = Math.floor(currentTime / 60);
//     console.log(`timeMinutes: ${timeMinutes}`);
//     let timeSeconds = Math.floor(currentTime % 60);
//     console.log(`timeSeconds: ${timeSeconds}`);
//     let timeSecondsFormatted = timeSeconds < 10 ? `0${timeSeconds}` : timeSeconds;
//     let currentTimeFormatted = `${timeMinutes}:${timeSecondsFormatted}`;
//     console.log(`currentTimeFormatted: ${currentTimeFormatted}`);
//     timeCurrent.innerHTML = currentTimeFormatted;
//   }
// }

// if (durationElement) {
//   durationElement.addEventListener('click', function(event) {
//     console.log(event.offsetX);
//     console.log(event.target);
//     let durationWidth = durationElement.offsetWidth;
//     console.log(durationWidth);
//     let durationPercentage = event.offsetX / durationWidth;
//     let selectedTime = duration * durationPercentage;
//     console.log(`Duration: ${duration}`);
//     console.log(`Selected time: ${selectedTime}`);
//     mediaElement.currentTime = selectedTime;
//   });
// }


// let fullScreenControl = document.querySelector('.li-control-fullscreen');
// if (fullScreenControl) {
//   fullScreenControl.addEventListener('click', toggleFullscreen);
// }
// let mediaPlayer = document.querySelector('.li-media-player');
// function toggleFullscreen() {
//   // Check if already in fullscreen
//   if (!document.fullscreenElement) {
//     console.log('Entering fullscreen');
//     mediaPlayer.requestFullscreen();
//   } else {
//     console.log('Exiting fullscreen');
//     document.exitFullscreen();
//   }

// }
</script>

<style lang="scss" scoped>
$control-element-spacer: 20px;
$control-element-button-size: 40px;
$control-background-color: #bbb;

.li-media-player {
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
  min-height: 86px;
  color: white;
  background: #ccc;
  font-family: sans-serif;
  font-size: 14px;
  user-select: none;
}
.li-media-player-controls {
  position: absolute;
  bottom: 0;
  top: 0;
  left: 0;
  right: 0;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
}
.li-control-row {
  display: flex;
  align-items: center;
  padding-top: $control-element-spacer / 2;
  &:last-child { padding-bottom: $control-element-spacer / 2; }
}
.li-control-fullscreen {
  margin-left: auto;
}
.li-player-volume {
  display: flex;
}
.li-volume {
  &-menu {
    display: flex;
    align-items: center;
    width: 80px;
    height: 40px;
    padding-left: 10px;
    padding-right: 10px;
    background: $control-background-color;
  }
  &-total {
    position: relative;
    height: 15px;
    width: 100%;
    background-color: #ccc;
  }
  &-amount {
    height: 100%;
    width: 25%;
    background: white;
  }
  &-handler {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
  }
}
.li-video-element {
  width: 100%;
}
.li-time {
  white-space: nowrap;
  margin-left: $control-element-spacer / 2;
  margin-right: $control-element-spacer;
}
.li-time-control {
  position: relative;
  width: 100%;
  height: 15px;
  margin-right: $control-element-spacer / 2;
  background: $control-background-color;
}
.li-progress {
  width: 0%;
  height: 100%;
  background: indianred;
}
.li-duration-handler {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}
.li-current-time {
  position: absolute;
  top: 0;
  left: 0%;
  width: 10px;
  height: 100%;
  transform: translateX(-50%);
  background: white;
}
</style>