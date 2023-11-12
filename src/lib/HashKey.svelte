<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { stored_key, decrypted, selected_account } from "../stores";

    import { toast } from 'svoast';
  
    // let password = '';
    let key = '';
    let decrypted_file = '';
  
    async function hash(){
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        // key = await invoke("hash_password", { password })
        // if (key === null) {
        //     key = "Error: Password is empty"
        // }
        toast.info('Checking...')
        decrypted_file = await invoke("get_decrypted_data", { keyFromUser: key })
        toast.success('Decrypted successfully!')
        stored_key.set(key);
        decrypted.set(JSON.parse(decrypted_file));
        console.log(decrypted_file);
        
        selected_account.set(-1)




    }
</script>
  
<div>
    <form class="row" on:submit|preventDefault={hash}>
        <input id="hash-input" placeholder="Master Password..." type="password" bind:value={key} />
        <button id="nav-submit">Submit</button>
    </form>
    <!-- <p>{key}</p> -->
</div>

<style lang="scss">
    #hash-input, #nav-submit {
        margin-right: 5px;
        // margin-top: 0.9em; 
    }

    input {
        padding-top: 0.5em;
        padding-bottom: 0.5em;
    }

    form {
        vertical-align: middle;
    }

</style>