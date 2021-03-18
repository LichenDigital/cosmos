<template lang="pug">
div.co-checkbox
  input(type='checkbox')
  coIcon.co-checkmark(:icon='checkmark')
</template>

<script lang="ts">

import { defineComponent } from 'vue';
import coIcon from './co-icon.vue';

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
  outline: none;
  border: 2px solid black;
  width: 2rem;
  height: 2rem;
  padding: 2px;
  background: white;
  margin-bottom: 1rem;

  input {
    position: absolute;
    top: 0; left: 0;
    cursor: pointer;
    width: 100%;
    height: 100%;
    opacity: 0;

    &:checked ~ .co-checkmark {
      opacity: 1;
    }

    &:focus {
      border-color: red;
    }
  }

  .co-checkmark {
    opacity: 0;
    pointer-events: none;
  }
}
</style>