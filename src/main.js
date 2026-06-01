const { invoke } = window.__TAURI__.core;
const { getCurrentWindow } = window.__TAURI__.window;
const { open } = window.__TAURI__.opener;

const appWindow = getCurrentWindow();

// Window controls
document.getElementById('btn-minimize').addEventListener('click', () => appWindow.minimize());

document.getElementById('btn-maximize').addEventListener('click', async () => {
  const isMax = await appWindow.isMaximized();
  isMax ? appWindow.unmaximize() : appWindow.maximize();
});

document.getElementById('btn-close').addEventListener('click', () => appWindow.close());

// Sesi baru
document.getElementById('btn-new-session').addEventListener('click', () => {
  invoke('new_tab').catch(err => showToast('Gagal membuka sesi: ' + err));
});

// Hapus cache
document.getElementById('btn-clear-cache').addEventListener('click', async () => {
  try {
    const result = await invoke('clear_cache');
    showToast(result);
  } catch (err) {
    showToast('Gagal hapus cache: ' + err);
  }
});

// About modal
const modal = document.getElementById('about-modal');

document.getElementById('btn-about').addEventListener('click', () => {
  modal.classList.remove('hidden');
});

document.getElementById('modal-close').addEventListener('click', () => {
  modal.classList.add('hidden');
});

modal.addEventListener('click', (e) => {
  if (e.target === modal) modal.classList.add('hidden');
});

document.getElementById('link-web').addEventListener('click', (e) => {
  e.preventDefault();
  open('https://ramdanolii.my.id');
});

document.getElementById('link-github').addEventListener('click', (e) => {
  e.preventDefault();
  open('https://github.com/ramdanolii14');
});

// Toast helper
function showToast(msg) {
  const toast = document.getElementById('toast');
  toast.textContent = msg;
  toast.classList.remove('hidden');
  setTimeout(() => toast.classList.add('hidden'), 3000);
}