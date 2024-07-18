using System;
using UnityEngine;
using UnityEngine.Serialization;

public class ResourceLoader : MonoBehaviour
{
    [Header( "Resources" )]
    public GameObject BattleUiPrefab;

    public GameObject OverworldUiPrefab;

    public GameObject StatScreenPrefab;

    public IntEventChannelSO IntEventChannelSO;

    [Header( "Data" )]
    public Game Game;
    
    [Header( "Listen to Event" )]
    [SerializeField] public VoidEventChannelSO m_LoadBattleUi;

    [SerializeField] public VoidEventChannelSO m_LoadOverworldUi;

    [SerializeField] private StringEventChannelSO m_LoadScene;

    private void Start()
    {
        DontDestroyOnLoad( gameObject );

        m_LoadBattleUi.OnEventRaised    += OnLoadBattleUi;
        m_LoadOverworldUi.OnEventRaised += OnLoadOverworldUi;
    }

    private void OnLoadBattleUi()
    {
        Instantiate( BattleUiPrefab );
    }

    private void OnLoadOverworldUi()
    {
        var oui = Instantiate( OverworldUiPrefab );
        oui.GetComponent< OverworldHUD >().observedEntity = Game.player.GetComponent< Entity >();
    }
}