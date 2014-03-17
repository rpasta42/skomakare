#include "ogre.h"

//OgreTaskQueue ogreTaskQueue;
//kk #include "OgreFramework.hpp"

using namespace Ogre;
using std::cout;
using std::endl;

template<> OgreFramework* Ogre::Singleton<OgreFramework>::msSingleton = 0;

OgreFramework::~OgreFramework()
{
    if (m_pInputMgr) OIS::InputManager::destroyInputSystem(m_pInputMgr);
    //kk if(m_pTrayMgr)  delete m_pTrayMgr;
    if (m_pRoot)     delete m_pRoot;
}

OgreFramework::OgreFramework()
{
  m_MoveSpeed			  = 3; //kk 0.1f;
  m_RotateSpeed     = 0.3f;

  m_bShutDownOgre   = false;
  m_iNumScreenShots = 0;

  m_pRoot           = 0;
  m_pSceneMgr       = 0;
  m_pRenderWnd      = 0;
  m_pCamera         = 0;
	m_pViewport       = 0;
	m_pLog            = 0;
	m_pTimer          = 0;

	m_pInputMgr       = 0;
	m_pKeyboard       = 0;
	m_pMouse          = 0;

        //kk m_pTrayMgr                      = 0;
        m_FrameEvent                    = Ogre::FrameEvent();
}


bool OgreFramework::initOgre(Ogre::String wndTitle, OIS::KeyListener *pKeyListener, OIS::MouseListener *pMouseListener)
{
	Ogre::LogManager* logMgr = new Ogre::LogManager();

	m_pLog = Ogre::LogManager::getSingleton().createLog("OgreLogfile.log", true, true, false);
	m_pLog->setDebugOutputEnabled(true);

	m_pRoot = new Ogre::Root();

	if(!m_pRoot->showConfigDialog())
		return false;
	m_pRenderWnd = m_pRoot->initialise(true, wndTitle);

	m_pSceneMgr = m_pRoot->createSceneManager(ST_GENERIC, "SceneManager");
	m_pSceneMgr->setAmbientLight(Ogre::ColourValue(0.7f, 0.7f, 0.7f));

	m_pCamera = m_pSceneMgr->createCamera("Camera");
	m_pCamera->setPosition(Vector3(0, 60, 60));
	m_pCamera->lookAt(Vector3(0, 0, 0));
	m_pCamera->setNearClipDistance(1);

	m_pViewport = m_pRenderWnd->addViewport(m_pCamera);
	m_pViewport->setBackgroundColour(ColourValue(0.8f, 0.7f, 0.6f, 1.0f));

	m_pCamera->setAspectRatio(Real(m_pViewport->getActualWidth()) / Real(m_pViewport->getActualHeight()));

	m_pViewport->setCamera(m_pCamera);

	size_t hWnd = 0;
        OIS::ParamList paramList;
        m_pRenderWnd->getCustomAttribute("WINDOW", &hWnd);

	paramList.insert(OIS::ParamList::value_type("WINDOW", Ogre::StringConverter::toString(hWnd)));

	m_pInputMgr = OIS::InputManager::createInputSystem(paramList);

        m_pKeyboard = static_cast<OIS::Keyboard*>(m_pInputMgr->createInputObject(OIS::OISKeyboard, true));
	m_pMouse = static_cast<OIS::Mouse*>(m_pInputMgr->createInputObject(OIS::OISMouse, true));

	m_pMouse->getMouseState().height = m_pRenderWnd->getHeight();
	m_pMouse->getMouseState().width	 = m_pRenderWnd->getWidth();

	if(pKeyListener == 0)
		m_pKeyboard->setEventCallback(this);
	else
		m_pKeyboard->setEventCallback(pKeyListener);

	if(pMouseListener == 0)
		m_pMouse->setEventCallback(this);
	else
		m_pMouse->setEventCallback(pMouseListener);

	Ogre::String secName, typeName, archName;
	Ogre::ConfigFile cf;
        cf.load("resources.cfg");

	Ogre::ConfigFile::SectionIterator seci = cf.getSectionIterator();
        while (seci.hasMoreElements())
        {
            secName = seci.peekNextKey();
		Ogre::ConfigFile::SettingsMultiMap *settings = seci.getNext();
            Ogre::ConfigFile::SettingsMultiMap::iterator i;
            for (i = settings->begin(); i != settings->end(); ++i)
            {
                typeName = i->first;
                archName = i->second;
                Ogre::ResourceGroupManager::getSingleton().addResourceLocation(archName, typeName, secName);
            }
        }
	Ogre::TextureManager::getSingleton().setDefaultNumMipmaps(5);
	Ogre::ResourceGroupManager::getSingleton().initialiseAllResourceGroups();

	m_pTimer = new Ogre::Timer();
	m_pTimer->reset();

  //kk
  OgreBites::InputContext *ic = new OgreBites::InputContext;
  ic->mMouse = m_pMouse;
  ic->mKeyboard = m_pKeyboard;
  //ic->capture();

	/*m_pTrayMgr = new OgreBites::SdkTrayManager("TrayMgr", m_pRenderWnd, *ic/*m_pMouse*//*, this);
        m_pTrayMgr->showFrameStats(OgreBites::TL_BOTTOMLEFT);
        m_pTrayMgr->showLogo(OgreBites::TL_BOTTOMRIGHT);
        m_pTrayMgr->hideCursor();

	m_pRenderWnd->setActive(true);*/

	return true;
}

bool OgreFramework::keyPressed(const OIS::KeyEvent &keyEventRef)
{
	if(m_pKeyboard->isKeyDown(OIS::KC_ESCAPE))
	{
			m_bShutDownOgre = true;
			return true;
	}

	if(m_pKeyboard->isKeyDown(OIS::KC_SYSRQ))
	{
		m_pRenderWnd->writeContentsToTimestampedFile("BOF_Screenshot_", ".png");
		return true;
	}

	if(m_pKeyboard->isKeyDown(OIS::KC_M))
	{
		static int mode = 0;

		if(mode == 2)
		{
			m_pCamera->setPolygonMode(PM_SOLID);
			mode = 0;
		}
		else if(mode == 0)
		{
			 m_pCamera->setPolygonMode(PM_WIREFRAME);
			 mode = 1;
		}
		else if(mode == 1)
		{
			m_pCamera->setPolygonMode(PM_POINTS);
			mode = 2;
		}
	}

	/*if(m_pKeyboard->isKeyDown(OIS::KC_O))
	{
		if(m_pTrayMgr->isLogoVisible())
                {
                        m_pTrayMgr->hideLogo();
                        m_pTrayMgr->hideFrameStats();
                }
                else
                {
                        m_pTrayMgr->showLogo(OgreBites::TL_BOTTOMRIGHT);
                        m_pTrayMgr->showFrameStats(OgreBites::TL_BOTTOMLEFT);
                }
	}*/

	return true;
}

bool OgreFramework::keyReleased(const OIS::KeyEvent &keyEventRef)
{
	return true;
}

bool OgreFramework::mouseMoved(const OIS::MouseEvent &evt)
{
	m_pCamera->yaw(Degree(evt.state.X.rel * -0.1f));
	m_pCamera->pitch(Degree(evt.state.Y.rel * -0.1f));

	return true;
}

bool OgreFramework::mousePressed(const OIS::MouseEvent &evt, OIS::MouseButtonID id)
{
	return true;
}

bool OgreFramework::mouseReleased(const OIS::MouseEvent &evt, OIS::MouseButtonID id)
{
	return true;
}

void OgreFramework::updateOgre(double timeSinceLastFrame)
{
	m_MoveScale = m_MoveSpeed   * (float)timeSinceLastFrame;
	m_RotScale  = m_RotateSpeed * (float)timeSinceLastFrame;

	m_TranslateVector = Vector3::ZERO;

	getInput();
	moveCamera();

	m_FrameEvent.timeSinceLastFrame = timeSinceLastFrame;
        //m_pTrayMgr->frameRenderingQueued(m_FrameEvent);
}

void OgreFramework::moveCamera()
{
	if(m_pKeyboard->isKeyDown(OIS::KC_LSHIFT))
		m_pCamera->moveRelative(m_TranslateVector);
	else
		m_pCamera->moveRelative(m_TranslateVector / 10);
}

void OgreFramework::getInput()
{
	if(m_pKeyboard->isKeyDown(OIS::KC_A))
		m_TranslateVector.x = -m_MoveScale;

	if(m_pKeyboard->isKeyDown(OIS::KC_D))
		m_TranslateVector.x = m_MoveScale;

	if(m_pKeyboard->isKeyDown(OIS::KC_W))
		m_TranslateVector.z = -m_MoveScale;

	if(m_pKeyboard->isKeyDown(OIS::KC_S))
		m_TranslateVector.z = m_MoveScale;
}



extern "C" {
#include "repl.h" /* contains main code for REPL */
#include "init.h"
#include "blend_format.h"
}
#include <pthread.h>


/* Windows crap:

#if OGRE_PLATFORM == PLATFORM_WIN32 || OGRE_PLATFORM == OGRE_PLATFORM_WIN32
  #define WIN32_LEAN_AND_MEAN
  #include "windows.h"
INT WINAPI WinMain(HINSTANCE hInst, HINSTANCE, LPSTR strCmdLine, INT)
#endif

replace frpintf with:
MessageBoxA(NULL, e.what(), "An exception has occurred!", MB_OK | MB_ICONERROR | MB_TASKMODAL);

replace sleep(1) with:
Sleep(1000);
*/
int main(int argc, char **argv) {
  /* Make a thread for REPL, and launch graphics in main thread. */
  pthread_t repl_thread;
  int ret_val = pthread_create(&repl_thread, NULL, repl, NULL);

  //ret_val++;
  /* Window which displays OpenGL drawings. */
  //init_graphics(nargs, args, "Skomakare", 500, 500);

	try {
    bool shutdown = false;

    new OgreFramework();
    OgreFramework* ogre = OgreFramework::getSingletonPtr();
    SkmkInput skmkInput(ogre);

    if(!ogre->initOgre("DemoApp v1.0", &skmkInput, 0))
      return;

    ogre->m_pLog->logMessage("Demo initialized!");

    SceneManager* mgr = ogre->m_pSceneMgr;
    Ogre::SceneNode* root = mgr->getRootSceneNode();


    //ogre->m_pSceneMgr->setSkyBox(true, "Examples/SpaceSkyBox");

    mgr->createLight("Light")->setPosition(75,75,75);

    Ogre::SceneNode* ogreHeadNode = root->createChildSceneNode("OgreHeadNode");
    Ogre::Entity* ogreHeadEntity = mgr->createEntity("OgreHeadEntity", "ogrehead.mesh"/*"sphere.mesh"*/);

    ogreHeadNode->attachObject(ogreHeadEntity);

    ogre->m_pLog->logMessage("Start main loop...");

    double timeSinceLastFrame = 0;
    double startTime = 0;

    ogre->m_pRenderWnd->resetStatistics();

    while (!shutdown && !ogre->isOgreToBeShutDown()) {

      if (ogre->m_pRenderWnd->isClosed())
        shutdown = true;

      Ogre::WindowEventUtilities::messagePump();

      if(ogre->m_pRenderWnd->isActive()) {
        startTime = ogre->m_pTimer->getMillisecondsCPU();

        ogre->m_pKeyboard->capture();
        ogre->m_pMouse->capture();

        ogre->updateOgre(timeSinceLastFrame);
        ogre->m_pRoot->renderOneFrame();

        timeSinceLastFrame = ogre->m_pTimer->getMillisecondsCPU() - startTime;
      }
      else
        sleep(1);
    }

    ogre->m_pLog->logMessage("Main loop quit");
    ogre->m_pLog->logMessage("Shutdown OGRE...");
    delete ogre;
  }
	catch(std::exception& e) {
    fprintf(stderr, "An exception has occurred: %s\n", e.what());
  }

  pthread_join(repl_thread, NULL);

  return 0;
}


#define skmk_try_start try {
#define skmk_try_end(action) \
} \
  catch (std::exception &e) {  \
  /* frpintf(stderr, "Exception"); */ \
    cout << "exception: " << e.what() << "\n function: " << __func__ << endl; \
    action  \
}


void* _ogre_get_scene_mgr() {
  skmk_try_start
  return (void*)(OgreFramework::getSingletonPtr()->m_pSceneMgr);
  skmk_try_end(return 0;)
}
void* _ogre_get_root_scene_node() {
  skmk_try_start
  return (void*)(((Ogre::SceneManager*)ogre_get_scene_mgr())->getRootSceneNode());
  skmk_try_end(return 0;)
}
void* _ogre_create_light(const char *name) {
  skmk_try_start
  return (void*)(((Ogre::SceneManager*)ogre_get_scene_mgr())->createLight(std::string(name)));
  skmk_try_end(return 0;)
}
void* _ogre_create_entity(const char *name, const char *mesh_path) {
  skmk_try_start
  return (void*)
    (((Ogre::SceneManager*)ogre_get_scene_mgr())->createEntity(std::string(name),
                                                               std::string(mesh_path)));
  skmk_try_end(return 0;)
}
void* _ogre_create_child_scene_node(void *parent, const char *name) {
  skmk_try_start
  return (void*)
    (((Ogre::SceneNode*)parent)->createChildSceneNode(std::string(name)));
  skmk_try_end(return 0;)
}
void _ogre_attach_object(void *parent, void *child) {
  skmk_try_start
  ((Ogre::SceneNode*)parent)->attachObject((Ogre::Entity*)child);
  skmk_try_end(;)
}
void _ogre_set_position(void* obj, float a, float b, float c) {
  skmk_try_start
  ((Ogre::SceneNode*)obj)->setPosition(a, b, c);
  skmk_try_end(;)
}



/* If Ogre3D is not compiled with multi-threaded support,
 * we cannot call ogre rendering functions from different threads.
 * This is a dirty hack to enable multiple threads to call
 * ogre functions, by queueing calls, and executing them in
 * "main thread" loop.
 */
#ifndef skmk_ogre_single_thread
void* ogre_get_scene_mgr() {
  return _ogre_get_scene_mgr();
  //ogreTaskQueue.add(OgreCall(OgreAPI::GET_SCENE_MGR, NULL));
}
void* ogre_get_root_scene_node() {
  return _ogre_get_root_scene_node();
}
void* ogre_create_light(const char *name) {
  return _ogre_create_light(name);
}
void* ogre_create_entity(const char *name, const char *mesh_path) {
  return _ogre_create_entity(name, mesh_path);
}
void* ogre_create_child_scene_node(void *parent, const char *name) {
  return _ogre_create_child_scene_node(parent, name);
}
void ogre_attach_object(void *parent, void *child) {
  _ogre_attach_object(parent, child);
}
void ogre_set_position(void* obj, float a, float b, float c) {
  _ogre_set_position(obj, a, b, c);
}
#else //skmk_ogre_single_thread


#endif //skmk_ogre_single_thread













