<template lang="pug">
div.co-icon
  svg
    use(:href='iconPath')
</template>

<script lang="ts">

import { defineComponent } from 'vue';

export default defineComponent({
  name: 'coIcon',
  props: {
    icon: {
      type: String,
      default: 'image'
    }
  },
  computed: {
    iconPath(): string {
      const path = '/img/cosmos-icons.svg';
      return `${path}#co-${this.icon}`;
    }
  }
});

</script>

<style lang="scss">
  .co-icon {
    height: 20px;
    width: 20px;
    svg {
      width: 100%;
      height: 100%;
      fill: black;
    }
  }
</style>