<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { ask } from "@tauri-apps/plugin-dialog";

    let fileName = "";
    let filePath = "";
    let password = "";

    async function pickFile() {
        const file = await open({
            multiple: false,
            directory: false,
        });

        if (file) {
            fileName = file.name as string;
            filePath = file.path;
        }
    }

    async function submitFileEncryption() {
        // TODO: save to pass password as a plain text via IPC?
        await invoke("encrypt_file", { path: filePath, password });
    }
</script>

<div>
    <button on:click={pickFile}>Pick file</button>
    <p>{fileName}</p>

    <form class="row" on:submit|preventDefault={submitFileEncryption}>
        <input
            id="password"
            placeholder="Encrypt with a password"
            bind:value={password}
        />
        <button type="submit">Encrypt</button>
    </form>
</div>
