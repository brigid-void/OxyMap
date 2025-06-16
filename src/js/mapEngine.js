// MapLibre GL JS integration
export function initMap(containerId, style) {
    return new maplibregl.Map({
        container: containerId,
        style: style,
        center: [0, 0],
        zoom: 1
    });
}
