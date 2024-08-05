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
        shift(type)
        {
            console.log(this.inputPath);
            invoke("oklab_shift", { inputSvgFile: this.inputPath, shiftType: type, shiftColor: this.colorPallet[0]})
            .then((res) => 
            {
                console.log(res);
            })
        },
    }
}
</script>