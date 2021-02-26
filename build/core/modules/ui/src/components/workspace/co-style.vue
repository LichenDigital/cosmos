<template>
    <div class="style">
        <div class="size">
            <co-input></co-input>

        </div>
        <div class="position">

        </div>
        <div class="effects">

        </div>
        <div class="typography">

        </div>
        <div class="spacing">

        </div>
        <div class="layout">

        </div>
        <div class="backgrounds">

        </div>
        <div class="borders"></div>
    </div>
</template>

<script>
import CoInput from '../co-input.vue';

export default {
    name: 'co-style',
    components: {
        'co-input': CoInput
    }
}
</script>


<style lang="scss" module>
.co-style {
    width: 40%;
    height: 100%;
    display: block;
}

</style>
