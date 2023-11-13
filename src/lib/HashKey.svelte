<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { stored_key, decrypted, selected_account } from "../stores";

    import { toast } from 'svoast';
  
    let key = '';
    let decrypted_file = '';
  
    async function hash(){
        toast.info('Checking...')
        decrypted_file = await invoke("get_decrypted_data", { keyFromUser: key })
        toast.success('Decrypted successfully!')
        
        stored_key.set(key);
        decrypted.set(JSON.parse(decrypted_file));
        
        selected_account.set(-1)
    }
</script>
  
<div>
    <form class="row" on:submit|preventDefault={hash}>
        <input id="hash-input" placeholder="Master Password..." type="password" bind:value={key} />
        <button id="nav-submit">Submit</button>
    </form>
</div>

<style lang="scss">
    #hash-input, #nav-submit {
        margin-right: 5px;
    }

    input {
        padding-top: 0.5em;
        padding-bottom: 0.5em;
    }

    form {
        vertical-align: middle;
    }

</style>