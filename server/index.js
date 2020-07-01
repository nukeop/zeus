import electron, { BrowserWindow } from 'electron';
import path from 'path';
import url from 'url';

const app = electron.app;
let win;

app.allowRendererProcessReuse = false;
function createWindow() {
  win = new BrowserWindow({
    width: 640,
    height: 480,
    show: false,
    webPreferences: {
      nodeIntegration: true
    }
  });

  win.setTitle('Zeus Entertainment Unit');

  win.loadURL(url.format({
    pathname: path.join('localhost:8080', 'index.html'),
    protocol: 'http:',
    slashes: true
  }));

  win.once('ready-to-show', () => {
    win.show();
  });

  win.on('closed', () => {
    win = null;
  });
}

app.on('ready', createWindow);

app.on('window-all-closed', () => {
  app.quit();
});
