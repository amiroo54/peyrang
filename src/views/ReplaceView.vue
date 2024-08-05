<template>
    <ColorList />
    <button @click="replace"></button>
</template>

<script>
import ColorList from '../components/ColorList.vue';
export default
{
    components: {ColorList},
    methods:
    {
        replace() {
            console.log(this.inputPath);
            invoke("replace_svg_color", { inputSvgFile: this.inputPath, colorPallete: this.colorPallet})
            .then((res) => 
            {
                console.log(res);
            })
        }
    }
}
</script>