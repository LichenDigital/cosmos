<template lang="pug">
.loading.laser
  .light
  .light
</template>

<script lang="ts">

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