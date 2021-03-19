<template lang="pug">
div.co-checkbox
  input(type='checkbox')
  coIcon.co-checkmark(:icon='checkmark')
</template>

<script lang="ts">

import { defineComponent } from 'vue';
import coIcon from '../co-icon.vue';

export default defineComponent({
  name: 'coCheckbox',
  components: {
    coIcon
  },
  props: {
    checkmark: {
      type: String,
      default: 'check'
    }
  }
});

</script>

<style lang="scss">
.co-checkbox {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
  padding: 4px;
  margin-bottom: 1rem;

  input {
    border: 2px solid black;
    width: 100%;
    height: 100%;
    background: white;
    appearance: none;
    outline: none;
    position: absolute;
    top: 0; left: 0;

    &:checked ~ .co-checkmark {
      opacity: 1;
    }

    &:focus {
      box-shadow: 0 0 0 2px #888;
    }
  }

  .co-checkmark {
      position: relative;
      opacity: 0;
      z-index: 1;
      pointer-events: none;
  }
}
</style>