<template lang="pug">
.co-workspace.co-group
  .co-panel-label workspace
</template>

<script lang="ts">
export default {
  name: 'workspace'
}
</script>


<style lang="scss" scoped>
  .co-workspace {
    width: 40%;
    background: #eee;
    @media (max-width: 31.25em) {
      width: 100%;
    }
  }
</style>
