// UI controls for map interaction
export function setupMapControls(map) {
    // Add navigation controls
    map.addControl(new maplibregl.NavigationControl());
    
    // Add geolocation control
    map.addControl(new maplibregl.GeolocateControl({
        positionOptions: { enableHighAccuracy: true },
        trackUserLocation: true
    }));
}
