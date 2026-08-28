import '@fontsource-variable/manrope/wght.css';
import './styles.css';

type Scenario = 'escape' | 'ignored' | 'metadata' | 'unsupported';

const scenarios: Record<Scenario, { command: string; status: string; rows: string[]; mode?: string; error?: boolean }> = {
  escape: {
    command: 'agent → write ~/.ssh/agent.conf',
    status: 'BLOCKED · operation not permitted',
    rows: ['×  child syscall   permission denied', '✓  persistent changes   0', '✓  worktree unchanged']
  },
  ignored: {
    command: 'agent → create target/cache.pyc',
    status: 'ALLOWED · included in receipt',
    rows: ['+  created   target/cache.pyc', '#  sha256   607de9…be42', '✓  ignored file reported']
  },
  metadata: {
    command: 'agent → modify .git/hooks/pre-commit',
    status: 'ALLOWED · included in receipt',
    rows: ['~  modified  .git/hooks/pre-commit', '~  mode      100644 → 100755', '✓  Git metadata reported']
  },
  unsupported: {
    command: 'awb → prepare Landlock rules',
    status: 'REFUSED · enforcement unavailable',
    rows: ['!  Landlock ABI 3+ required', '×  command was not started', '→  use a supported kernel or explicit audit mode'],
    mode: 'FAILED CLOSED',
    error: true
  }
};

const runButton = document.querySelector<HTMLButtonElement>('#run-demo');
const result = document.querySelector<HTMLDivElement>('#demo-result');
const empty = document.querySelector<HTMLDivElement>('#demo-empty');
const status = document.querySelector<HTMLParagraphElement>('#demo-status');
const terminalMode = document.querySelector<HTMLElement>('.terminal-mode');

runButton?.addEventListener('click', () => {
  const choice = document.querySelector<HTMLInputElement>('input[name="scenario"]:checked');
  const scenario = scenarios[(choice?.value ?? 'escape') as Scenario];
  runButton.disabled = true;
  runButton.firstChild!.textContent = 'Checking boundary ';
  empty?.setAttribute('hidden', '');
  if (result) {
    result.hidden = false;
    result.innerHTML = '<p class="checking">Checking policy and snapshot…</p>';
  }
  if (status) status.textContent = 'Checking the selected write against policy.';

  window.setTimeout(() => {
    if (terminalMode) {
      terminalMode.textContent = scenario.mode ?? 'ENFORCED';
      terminalMode.classList.toggle('mode-error', Boolean(scenario.error));
    }
    if (result) {
      result.innerHTML = `
        <p class="result-command"><span>›</span> ${scenario.command}</p>
        <p class="result-status ${scenario.error ? 'is-error' : ''}">${scenario.status}</p>
        <div class="receipt-lines">${scenario.rows.map((row) => `<code>${row}</code>`).join('')}</div>
        <p class="receipt-path">receipt <span>.awb/receipts/01J6…9J.json</span></p>`;
    }
    if (status) status.textContent = scenario.status;
    runButton.disabled = false;
    runButton.firstChild!.textContent = 'Run simulation ';
  }, window.matchMedia('(prefers-reduced-motion: reduce)').matches ? 0 : 420);
});

document.querySelectorAll<HTMLButtonElement>('[data-copy]').forEach((button) => {
  button.addEventListener('click', async () => {
    const target = document.getElementById(button.dataset.copy ?? '');
    if (!target) return;
    try {
      await navigator.clipboard.writeText(target.textContent ?? '');
      const original = button.textContent;
      button.textContent = 'Copied';
      window.setTimeout(() => { button.textContent = original; }, 1600);
    } catch {
      button.textContent = 'Select command to copy';
      const selection = window.getSelection();
      const range = document.createRange();
      range.selectNodeContents(target);
      selection?.removeAllRanges();
      selection?.addRange(range);
    }
  });
});

const offlineBar = document.querySelector<HTMLElement>('#offline-bar');
const updateConnection = () => {
  if (offlineBar) offlineBar.hidden = navigator.onLine;
};
window.addEventListener('online', updateConnection);
window.addEventListener('offline', updateConnection);
updateConnection();

if ('serviceWorker' in navigator && import.meta.env.PROD) {
  void navigator.serviceWorker.register('/sw.js', { updateViaCache: 'none' });
}
