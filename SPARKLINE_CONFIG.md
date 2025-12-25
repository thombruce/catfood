# Sparkline Configuration

Catfood's sparkline components (CPU, RAM, WiFi) now support logarithmic scaling for better visualization of data with high dynamic range.

## Configuration Options

All sparkline-enabled components support these configuration options:

```json
{
  "name": "component_name",
  "sparkline": true,                           // Enable sparkline
  "sparkline_length": 15,                      // Number of characters in sparkline
  "sparkline_update_freq": 2,                  // Update frequency in seconds
  "sparkline_logarithmic": false               // Use logarithmic scaling (optional)
}
```

## Component Defaults

### CPU Component
- **Default scaling**: Linear (`sparkline_logarithmic: false`)
- **Range**: 0-100% CPU usage
- **Use case**: Linear scaling works well for percentages since the range is bounded

### RAM Component  
- **Default scaling**: Linear (`sparkline_logarithmic: false`)
- **Range**: 0-100% memory usage
- **Use case**: Linear scaling works well for percentages since the range is bounded

### WiFi Component
- **Default scaling**: Logarithmic (`sparkline_logarithmic: true`)
- **Range**: Variable network throughput (bytes to megabytes)
- **Use case**: Logarithmic scaling shows both background activity and large transfers

## Linear vs Logarithmic Scaling

### Linear Scaling (Default for CPU/RAM)
```
Data:    [0, 10, 25, 50, 75, 100]
Display: [  , ▁ , ▂ , ▄ , ▆ , █ ]
```
- Equal percentage changes have equal visual impact
- Best for bounded ranges like percentages

### Logarithmic Scaling (Default for WiFi)
```
Data:    [1, 10, 100, 1000, 10000, 100000]
Display: [  , ▁ , ▂ , ▃ , ▄ , ▅ ]
```
- Equal ratio changes have equal visual impact
- Best for data with high dynamic range (network traffic, file sizes)
- Small values remain visible even when large values are present

## Configuration Examples

### CPU with Linear Scaling (Default)
```json
{
  "name": "cpu",
  "sparkline": true,
  "sparkline_length": 10,
  "sparkline_update_freq": 3
}
```

### WiFi with Logarithmic Scaling (Default)
```json
{
  "name": "wifi", 
  "sparkline": true,
  "sparkline_length": 15,
  "sparkline_update_freq": 2
}
```

### Override Default: CPU with Logarithmic Scaling
```json
{
  "name": "cpu",
  "sparkline": true,
  "sparkline_length": 10,
  "sparkline_update_freq": 3,
  "sparkline_logarithmic": true
}
```

### Override Default: WiFi with Linear Scaling
```json
{
  "name": "wifi",
  "sparkline": true,
  "sparkline_length": 15,
  "sparkline_update_freq": 2,
  "sparkline_logarithmic": false
}
```

## Benefits of Logarithmic Scaling for WiFi

With logarithmic scaling enabled for WiFi sparklines:

1. **Background activity visibility**: Small network traffic (like background syncs) remains visible
2. **Large activity handling**: Large file downloads don't completely drown out normal usage
3. **Better dynamics**: More useful visualization across different usage patterns
4. **Zero handling**: Zero values still display as empty space for clean visuals

## Backward Compatibility

The new `sparkline_logarithmic` option is completely optional:
- Existing configurations continue to work unchanged
- Component defaults ensure optimal behavior without configuration
- Users can explicitly set the option based on their preferences