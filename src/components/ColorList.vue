<template>

    <div 
    v-for="color in colorPallet" 
    :key="color" 
    :class="(activeRequired && colorPallet.indexOf(color) == active)? 'selected':'unselected'">

        <color-picker format="hex" 
        @update:pureColor="$emit('updateColor', color)" 
        :pureColor="color"/>


    </div>

    <button v-if="addRemoveButtons" @click="addcolor">Add Color</button>
    <button v-if="addRemoveButtons" @click="removeColor">Remove Color</button>
</template>

<script>

export default
{
    data() { 
        return {
            colorPalette: [],
            active: 0,
            activeRequired: false,
            addRemoveButtons: false,
        }
    },
    emits: ["updateColor"],
    methods: {
        addColor()
        {
            this.colorPalette.push("#FFFFFF");
        },
        removeColor()
        {
            const index = this.colorPalette.indexOf(this.active);
            this.colorPalette.splice(index, 1);
        }
    }
}
</script>