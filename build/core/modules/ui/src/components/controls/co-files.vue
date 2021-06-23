<template lang="pug">
.container
  .file-management
    coButton Upload
    coButton Edit
    coDropdown
  .files
    .file(v-for="file, index in files" :key="index")
      .preview
      .info
        .item {{ file.name }}
        .item {{ file.size }}
        coProgress(:bars='[[0, "40%"]]' :range='200')
</template>

<script lang="ts">

// Need to think about the functionality of a file browser... Uploading files maybe should just be thought of the functionality of uploading a file to another location and organizing the files from there. Aka being able to add files, reorder them edit their data (like name etc.), and remove them. Yes. this is the ticket.

import { defineComponent } from 'vue';
import coIcon from '../co-icon.vue';
import coProgress from '../co-progress.vue';
import coButton from './co-button.vue';
import coDropdown from './co-dropdown.vue';

export default defineComponent({
  props: {

  },
  name: 'upload',
  components: {
    coIcon,
    coProgress,
    coButton,
    coDropdown
  },
  data() {
    return {
      files: [
        {
          name: 'File 1',
          size: '48KB',
          data: ''
        },
        {
          name: 'File 1',
          size: '48KB',
          data: ''
        },
        {
          name: 'File 2',
          size: '78KB',
          data: ''
        }
      ]
    }
  }
});

</script>

<style lang="scss" scoped>
  .container {
    width: 100%;
    background-color: #ddd;
    padding: 1rem;
  }
  .file-management {
    width: 100%;
    display: flex;
    align-items: flex-start;
    justify-content: flex-start;
    margin-bottom: 1rem;
  }
  .files {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-auto-columns: 1fr;
    grid-auto-rows: 1fr;
    column-gap: 1rem;
    row-gap: 1rem; 
  }
  .file {
    background-color: #bbb;
  }
  .preview {
    background-image: url(https://homepages.cae.wisc.edu/~ece533/images/airplane.png);
    background-size: cover;
    background-position: center;
    width: 100%;
    height: 10rem;
  }
  .info {
    padding: .5rem;
    display: flex;
    flex-direction: column;
    position: relative;
  }
  .item {
  }

</style>