<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

import FontSection from "./components/FontSection.vue";
import ColorSection from "./components/ColorSection.vue";
import LayoutSection from "./components/LayoutSection.vue";
import FormatSection from "./components/FormatSection.vue";

interface StyleConfig {
  font: Record<string, string>;
  color: Record<string, string>;
  layout: Record<string, string>;
  format: Record<string, string>;
}

const currentFilePath = ref("");
const config = reactive<StyleConfig>({
  font: {},
  color: {},
  layout: {},
  format: {}
});

const defaultConfig = reactive<StyleConfig>({ font: {}, color: {}, layout: {}, format: {} });

const isLoading = ref(false);
const saveStatus = ref("");
const showStatus = ref(false);

// フォントファミリーの選択肢
const fontFamilies = ref<string[]>([]);

onMounted(async () => {
  await loadDefaultConfig();
  await loadSystemFonts();
});

async function loadDefaultConfig() {
  try {
    isLoading.value = true;
    const defaultPath = await invoke<string>("get_default_style_path");
    const userPath = await invoke<string>("get_user_style_path");
    // デフォルト値も取得
    const def = await invoke<StyleConfig>("load_style_config", { filePath: defaultPath });
    Object.assign(defaultConfig, def);
    // ユーザー設定ファイルが存在する場合はそちらを優先
    try {
      const userConfig = await invoke<StyleConfig>("load_style_config", { filePath: userPath });
      Object.assign(config, userConfig);
      currentFilePath.value = userPath;
      showMessage("ユーザー設定ファイルを読み込みました", "success");
    } catch {
      // ユーザー設定ファイルが存在しない場合はデフォルトファイルを読み込み
      Object.assign(config, def);
      currentFilePath.value = defaultPath;
      showMessage("デフォルト設定ファイルを読み込みました", "info");
    }
  } catch (error) {
    showMessage(`設定ファイルの読み込みに失敗しました: ${error}`, "error");
  } finally {
    isLoading.value = false;
  }
}

async function loadConfig() {
  try {
    isLoading.value = true;
    const configData = await invoke<StyleConfig>("load_style_config", { filePath: currentFilePath.value });
    Object.assign(config, configData);
    showMessage("設定ファイルを読み込みました", "success");
  } catch (error) {
    showMessage(`設定ファイルの読み込みに失敗しました: ${error}`, "error");
  } finally {
    isLoading.value = false;
  }
}

async function saveAsUserConfig() {
  try {
    isLoading.value = true;
    const userPath = await invoke<string>("get_user_style_path");
    await invoke("save_style_config", { 
      filePath: userPath, 
      config: config 
    });
    currentFilePath.value = userPath;
    showMessage("ユーザー設定ファイルとして保存しました", "success");
  } catch (error) {
    showMessage(`設定ファイルの保存に失敗しました: ${error}`, "error");
  } finally {
    isLoading.value = false;
  }
}

async function resetToDefault() {
  try {
    isLoading.value = true;
    const defaultPath = await invoke<string>("get_default_style_path");
    const defaultConfig = await invoke<StyleConfig>("load_style_config", { filePath: defaultPath });
    Object.assign(config, defaultConfig);
    showMessage("デフォルト設定にリセットしました", "success");
  } catch (error) {
    showMessage(`リセットに失敗しました: ${error}`, "error");
  } finally {
    isLoading.value = false;
  }
}

function showMessage(message: string, type: "success" | "error" | "info" = "info") {
  saveStatus.value = message;
  showStatus.value = true;
  // typeパラメータを使用してメッセージの種類を記録（将来的な拡張用）
  console.log(`Message (${type}): ${message}`);
  setTimeout(() => {
    showStatus.value = false;
  }, 3000);
}

async function openFileLocation() {
  if (currentFilePath.value) {
    try {
      await invoke("open_file_location", { filePath: currentFilePath.value });
    } catch (error) {
      showMessage(`ファイルを開けませんでした: ${error}`, "error");
    }
  }
}

async function loadSystemFonts() {
  try {
    const systemFonts = await invoke<string[]>("get_system_fonts");
    fontFamilies.value = systemFonts;
  } catch (error) {
    console.warn("システムフォントの取得に失敗しました:", error);
    // フォールバック用のデフォルトフォント
    fontFamilies.value = [
      "Yu Gothic UI",
      "Meiryo",
      "MS Gothic",
      "Consolas",
      "Courier New",
      "Arial",
      "Helvetica",
      "Segoe UI",
      "Tahoma",
      "Verdana"
    ];
  }
}
</script>

<template>
  <div class="app-container">
    <header class="app-header">
      <h1>AviUtl2 Style Editor</h1>
      <div class="header-controls">
        <button @click="loadConfig" :disabled="isLoading" class="btn btn-secondary">
          {{ isLoading ? '読み込み中...' : '再読み込み' }}
        </button>
        <button @click="saveAsUserConfig" :disabled="isLoading" class="btn btn-success">
          保存
        </button>
        <button @click="resetToDefault" :disabled="isLoading" class="btn btn-warning">
          リセット（デフォルトに戻す）
        </button>
        <button @click="openFileLocation" class="btn btn-info">
          ファイル場所を開く
        </button>
      </div>
    </header>

    <!-- トースト通知 -->
    <transition name="toast-fade">
      <div v-if="showStatus" class="toast" :class="saveStatus.includes('失敗') ? 'error' : 'success'">
        <span class="toast-icon">
          <svg v-if="saveStatus.includes('失敗')" width="20" height="20" viewBox="0 0 20 20" fill="none"><circle cx="10" cy="10" r="10" fill="#fff"/><path d="M6 6l8 8M14 6l-8 8" stroke="#dc3545" stroke-width="2" stroke-linecap="round"/></svg>
          <svg v-else width="20" height="20" viewBox="0 0 20 20" fill="none"><circle cx="10" cy="10" r="10" fill="#fff"/><path d="M6 10l3 3 5-5" stroke="#28a745" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
        </span>
        <span class="toast-message">{{ saveStatus }}</span>
      </div>
    </transition>

    <div class="file-info">
      <p><strong>現在のファイル:</strong> {{ currentFilePath }}</p>
    </div>

    <div class="main-content">
      <div class="tabs">
        <button 
          v-for="tab in ['Font', 'Color', 'Layout', 'Format']" 
          :key="tab"
          @click="activeTab = tab"
          :class="['tab-button', { active: activeTab === tab }]"
        >
          {{ tab }}
        </button>
      </div>

      <div class="tab-content">
        <FontSection 
          v-if="activeTab === 'Font'" 
          v-model="config.font" 
          :font-families="fontFamilies"
          :default-value="defaultConfig.font"
        />
        <ColorSection 
          v-if="activeTab === 'Color'" 
          v-model="config.color"
          :default-value="defaultConfig.color"
        />
        <LayoutSection 
          v-if="activeTab === 'Layout'" 
          v-model="config.layout"
          :default-value="defaultConfig.layout"
        />
        <FormatSection 
          v-if="activeTab === 'Format'" 
          v-model="config.format"
          :default-value="defaultConfig.format"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
const activeTab = ref('Font');
</script>

<style scoped>
.app-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 20px;
  border-bottom: 2px solid #e0e0e0;
}

.app-header h1 {
  margin: 0;
  color: #333;
  font-size: 2rem;
}

.header-controls {
  display: flex;
  gap: 10px;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background-color: #007bff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: #0056b3;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background-color: #545b62;
}

.btn-success {
  background-color: #28a745;
  color: white;
}

.btn-success:hover:not(:disabled) {
  background-color: #1e7e34;
}

.btn-info {
  background-color: #17a2b8;
  color: white;
}

.btn-info:hover:not(:disabled) {
  background-color: #117a8b;
}

.status-message {
  padding: 10px;
  margin-bottom: 20px;
  border-radius: 4px;
  text-align: center;
  font-weight: 500;
}

.status-message.success {
  background-color: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.status-message.error {
  background-color: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

.file-info {
  background-color: #f8f9fa;
  padding: 15px;
  border-radius: 4px;
  margin-bottom: 20px;
  border-left: 4px solid #007bff;
}

.file-info p {
  margin: 0;
  font-family: 'Consolas', monospace;
  font-size: 14px;
}

.main-content {
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.tabs {
  display: flex;
  background-color: #f8f9fa;
  border-bottom: 1px solid #dee2e6;
}

.tab-button {
  padding: 12px 24px;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  color: #6c757d;
  transition: all 0.2s;
}

.tab-button:hover {
  background-color: #e9ecef;
  color: #495057;
}

.tab-button.active {
  background-color: white;
  color: #007bff;
  border-bottom: 2px solid #007bff;
}

.tab-content {
  padding: 20px;
  min-height: 400px;
}

.toast {
  position: fixed;
  top: 32px;
  right: 32px;
  min-width: 220px;
  max-width: 400px;
  z-index: 9999;
  background: linear-gradient(120deg, #333 60%, #444 100%);
  color: #fff;
  padding: 16px 24px 16px 16px;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.18);
  font-size: 16px;
  font-weight: 500;
  opacity: 0.97;
  display: flex;
  align-items: center;
  gap: 12px;
  pointer-events: none;
}
.toast.success {
  background: linear-gradient(120deg, #28a745 60%, #218838 100%);
}
.toast.error {
  background: linear-gradient(120deg, #dc3545 60%, #b52a37 100%);
}
.toast-icon {
  display: flex;
  align-items: center;
}
.toast-message {
  flex: 1;
  word-break: break-word;
}
.toast-fade-enter-active, .toast-fade-leave-active {
  transition: opacity 0.4s, transform 0.4s;
}
.toast-fade-enter-from, .toast-fade-leave-to {
  opacity: 0;
  transform: translateY(-30px) scale(0.95);
}
.toast-fade-enter-to, .toast-fade-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}
@media (max-width: 600px) {
  .toast {
    right: 8px;
    left: 8px;
    top: 16px;
    min-width: unset;
    max-width: unset;
    font-size: 15px;
    padding: 12px 12px 12px 10px;
  }
}
</style>