<template lang="pug">
.loading.laser
  .light
  .light
</template>

<script lang="ts">
// Think that progress should have a loading status, so that progress bars can indicate loading of something and then switch to a determined progress when that happens.

import { defineComponent } from 'vue';

export default defineComponent({
  name: 'coLoading',
  data () {
    return {

    };
  }
});

</script>

<style lang="scss" scoped>
  .light {
    background: red;
  }
</style>