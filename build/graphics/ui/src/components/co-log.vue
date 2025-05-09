<template lang="pug">
.co-log(v-if='logEnabled')
  .co-log-status log level: {{ logLevel }} #[span(@click='showLog = !showLog') show log v]
  .co-log-data(v-if='showLog')
    .co-log-item(v-for='item, index in logData' :key='index') {{ item }}
</template>

<script lang="ts">
// import { log, logLevel, logEnabled } from './cosmos';
// log('verbose', 'this is a verbose message');
// log('normal', 'this is a normal message');
// log('test', 'this shouldn\'t appear since test is in the log levels array');
// log('extraverbose', 'IT\'S OVER 9000!!!');

import { defineComponent } from 'vue';
import coButton from './controls/co-button.vue';

export default defineComponent({
  name: 'log',
  components: {
    coButton
  },
  data() {
    return {
      showLog: false,
    };
  },
  computed: {
    logEnabled() {
      return this.$store.state.log.enabled;
    },
    logLevel() {
      return this.$store.state.log.level;
    },
    logData() {
      return this.$store.state.log.data;
    }
  }
});
</script>

<style lang="scss">
$color: #666;

.co-log {
  position: fixed;
  top: 1rem;
  right: 1rem;
  z-index: 9000;
  background: white;
  color: $color;
  font-weight: bold;
  &-status {
    padding: .2rem 1rem;
    border: 2px dashed $color;
    user-select: none;
    span {
      border-left: dotted $color 2px;
      padding-left: 1rem;
      margin-left: 1rem;
      cursor: pointer;
    }
  }
  &-data {
    border: dashed $color 2px;
    border-top: unset;
  }
  &-item {
    padding: .5rem 1rem;
    border-bottom: 2px solid $color;
    &:last-child {
      border-bottom: unset;
    }
  }
}
</style>