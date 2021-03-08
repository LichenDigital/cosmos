<template lang="pug">
.co-components.co-group
  h2.co-panel-label components
  h3 button
  coButton(:link='{destination: "https://lichen.co"}') button
  h3 textinput
  h3 checkbox
  h3 radiobutton
  h3 slider
  h3 progress
  coProgress
  h3 upload
  h3 link
  h3 datepicker
  h3 calendar
  h3 colorpicker
  h3 graph
  h3 heading
  h3 paragraph
  h3 link
  h3 quote
  h3 list
  h3 table
  h3 icon
  h3 image
  h3 video
  h3 audio
  h3 thumbnail
  h3 dropdown
  h3 navigation
  h3 map


</template>

<script lang="ts">

import coButton from '../co-button.vue';
import coProgress from '../co-progress.vue';

export default {
  name: 'components',
  components: {
    coButton,
    coProgress
  }
}
</script>

<style lang="scss" scoped>
.co-components {
  width: 35%;
  background-color: #e1e1e1;
}
</style>
