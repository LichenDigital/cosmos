<template lang="pug">
    .co(:id='id'  v-on:click='change($event)')
        .editor(:style='editor' :class="$style.editor")
            label width
                input(v-model="att.width")
            label height
                input(v-model="att.height")
            label background
                input(v-model="att.background")
        div.container(:class="$style.container")
            div(v-bind:style="test" data-co-group='buttons' data-co-id="12hdsf721o0d8gh12")        
            div(v-bind:style="test")        
            div(v-bind:style="test")        
            div(v-bind:style="test")   
</template>

<script>

// Get closest parent that matches a selector
// function closestParent( element, selector) {
// }

// Function to operate on the viewport
// Get attributes
// Set attributes

export default {
    data() {
        return {
            // Will eventually be the id of this unique ui component
            id: 'test-id',
            att: {
                width: '203px',
                height: '100px',
                background: 'indianred'
            },
            editorPosition: {
                top: 0,
                left: 0
            }
        }
    },
    methods: {
        goto() {
            location = this.link;
        },
        change(event) {
            let target = event.target;
            // Used to get the objects position relative to the VIEWPORT no the page
            // let bounds = event.target.getBoundingClientRect()
            let bounds = {
                top: target.offsetTop,
                left: target.offsetLeft,
                width: target.offsetWidth,
                height: target.offsetHeight
            }

            // If something other than the editor is being clicked on, move the editor to that position
            if (target) {
                // Check if the position desired is out of the viewport, if it is, move the editor to the closest position to it while keeping it inside the viewport
                this.editorPosition.left = bounds.left + bounds.width;
                this.editorPosition.top = bounds.top;
            }
        }
    },
    computed: {
        test() {
            return {
                width: this.att.width,
                height: this.att.height,
                background: this.att.background,
                transition: 'all .3s ease-in-out',
                margin: '10px',
                'flex-shrink': 0
            }
        },
        editor () {
            return {
                opacity: '1',
                position: 'absolute',
                top: this.editorPosition.top + 'px',
                left: this.editorPosition.left + 'px'
            }
        }
    }
}
</script>

<style scoped>
.editor {
    border: 1px solid rgba(0,0,0,.8);
    border-radius: 4px;
    background: white;
    padding: 20px;
    transition: top .5s, left .5s;
    transition-timing-function: ease-in-out;
}

</style>