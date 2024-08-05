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
        pallet() {
            console.log(this.inputPath);
            invoke("generate_svg_with_color_combinations", { inputSvgFile: this.inputPath, colorPallete: this.colorPallet})
            .then((res) => 
            {
                console.log(res);
                for (let out in res)
                {
                console.log(res[out]);
                const asset = convertFileSrc(res[out]);
                console.log(asset);
                this.output.push(asset); 
                }
            });
        },
    }
}
</script>