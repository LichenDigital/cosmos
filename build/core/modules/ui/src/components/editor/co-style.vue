<template lang="pug">
.co-style.co-group
  .co-panel-label style
  textarea.co-style-editor(placeholder='style here...')
  //- .co-property-object
  //- .co-option-group
  //-   .c0-option-group
  //-   h3 size
  //-   co-input
  //- .position
  //- .effects
  //- .typography
  //- .spacing
  //- .layout
  //- .backgrounds
  //- .borders
</template>

<script lang="ts">
import CoInput from '../co-input.vue';

export default {
  name: 'co-style',
  components: {
    'co-input': CoInput
  }
}
</script>


<style lang="scss" scoped>
.co-style {
  width: 25%;
  background: #444;
}

.co-style-editor {
  color: white;
  background: none;
  border: none;
  outline: none;
  height: 100%;
  min-width: 100%;
  max-width: 100%;
}
</style>
