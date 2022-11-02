<script setup>

defineProps({
    message: {
        id: Number,
        message: String,
    }
})

</script>


<template>
    <q-card>
        <q-card-section>
            {{ message }}
        </q-card-section>
    </q-card>
</template>