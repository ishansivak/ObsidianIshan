# Athena: Dashboard Layout

## Structure
```html
<div class="bg-glass border-glass shadow-glass p-4 rounded-lg min-h-screen flex">
  <!-- Sidebar -->
  <div class="w-64">
    [[Sidebar]]
  </div>
  
  <!-- Main Content -->
  <div class="flex-1 flex flex-col">
    [[Header]]
    <main class="p-4">
      <!-- Dashboard Content -->
      <div class="bg-glass border-glass shadow-glass p-4 rounded-lg">
        <h2>Dashboard Overview</h2>
        <p>Main content area.</p>
      </div>
    </main>
  </div>
</div>
```
