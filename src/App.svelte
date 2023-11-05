<script lang="ts">
    import NavBar from './lib/NavBar.svelte';
    import MainArea from './lib/MainArea.svelte';

    import { Toasts } from 'svoast';

</script>

<Toasts />

<main class="container">
    <NavBar />
    <!-- <h1>Welcome to Tauri!</h1> 

    <div class="row">
        <ExampleLogos />
    </div> 


    <p>
        Click on the Tauri, Vite, and Svelte logos to learn more.
    </p>

    <div class="row">
        <Greet />
    </div>
    
    <div class="row">
        <HashKey />
    </div>

    <div class="row">
        <StartUpData />
    </div> -->

    <MainArea />
        
  


</main>

<style>
    
</style>