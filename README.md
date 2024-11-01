# OpenEmergencyConnect

An open-source alternative to GoodSAM's emergency services platform. GoodSAM should be open source - it's quite simple to stream data and emergency services should not have to pay a premium. This project aims to create an open source version from scratch, using open source tools.

## Initial Goals

### Phase 1
- **Instant Video Streaming**
  - Enable emergency dispatchers to instantly access caller's camera feed
  - No app installation required for callers
  - Maintain audio call while streaming video
  - WebRTC-based solution for low-latency streaming

- **Instant Location Services** 
  - Real-time caller location tracking
  - Support for multiple coordinate systems:
    - Latitude/Longitude
    - Easting/Northing
    - What3Words integration
  - Direction and speed monitoring
  - Accuracy radius indication

## Future Goals
- Instant chat capabilities
- Media upload functionality
- Multi-agency collaboration tools
- Video storage and retention
- Translation services
- Vital signs monitoring via AI

## Technology Stack
- WebRTC for video streaming
- OpenStreetMap for mapping
- Node.js backend
- Progressive Web App (PWA) frontend
- WebSocket for real-time communications

## Contributing
This is an open source project and contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Why Open Source?
Emergency services worldwide rely on expensive proprietary solutions for video streaming and location services. This project aims to provide a free, open alternative that any emergency service can deploy and customize to their needs.

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Project Status
🚧 Currently in early development - Phase 1 implementation

## Get Involved
- Star and watch this repository
- Join our [Discord community](link-to-discord)
- Check out our [Issues](link-to-issues) page for tasks you can help with
- Read our [Documentation](link-to-docs) to learn more

## Contact
- Project Lead: [Your Name]
- Email: [Your Email]
- Twitter: [@handle]


- build with docker build -t oec-server .
-run with docker run -p 8080:8080 oec-server
