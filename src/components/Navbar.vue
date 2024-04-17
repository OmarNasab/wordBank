<script setup>

</script>

<template>
  <div class="flex w-full text-center border rounded border-blue-500">
    <router-link class="px-4 py-2 w-full  hover:bg-blue-100"
                 active-class="bg-blue-500 hover:bg-blue-800 hover:border-blue-800 text-white  " to="/noun">
      Nomen
    </router-link>
    <router-link class="px-4 py-2 w-full hover:bg-blue-100 "
                 active-class="bg-blue-500 hover:border-blue-800 hover:!bg-blue-800 text-white" to="/verb">Verben
    </router-link>
    <router-link class="px-4 py-2 w-full hover:bg-blue-100"
                 active-class="bg-blue-500 hover:border-blue-800 hover:!bg-blue-800 text-white"
                 to="/phrase">Phrase
    </router-link>
    <router-link class="px-4 py-2 w-full hover:bg-blue-100"
                 active-class="bg-blue-500 hover:border-blue-800 hover:!bg-blue-800 text-white"
                 to="/medical-phrases">Medical Phrase
    </router-link>
  </div>
</template>

